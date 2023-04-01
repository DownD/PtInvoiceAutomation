import argparse
import csv
import logging
import os
from dataclasses import dataclass, fields
from datetime import date, datetime
from typing import Optional

import cv2
import numpy as np
import subsetsum
from pdf2image import convert_from_bytes, convert_from_path
from qreader import QReader

LOGGER = logging.getLogger(__name__)

@dataclass
class QRCode:
    data: str
    bbox: list
    bbox_color: tuple
    title: str
    img: np.ndarray


@dataclass
class InvoiceQRCode():
    nif_emissor : int
    nif_comprador : int
    pais_comprador : str
    documento_tipo : str #Usually FT
    documento_estado : str #N?
    documento_data : date #YYYYMMDD
    documento_numero : str 
    atcud : str
    espaco_fiscal : str
    total_impostos : float
    total : float
    hash_suffix : str
    cert_numero : int
    info_adicional : Optional[str]
    qr_info : QRCode

    def __init__(self, code: QRCode):
        self.qr_info = code
        
        # Parse data
        tokens: list[str] = code.data.split("*")
        key_value_dict = {token.split(":")[0]:token.split(":")[1] for token in tokens}
        self.__set_fields(key_value_dict)

        code.title = self.atcud

    def __set_fields(self, data: dict[str,str]) -> None:
        self.nif_emissor = int(data["A"])
        self.nif_comprador = int(data["B"])
        self.pais_comprador = data["C"]
        self.documento_tipo = data["D"]
        self.documento_estado = data["E"]
        self.documento_data = datetime.strptime(data["F"], '%Y%m%d').date()
        self.documento_numero = data["G"]
        self.atcud = data["H"]
        self.espaco_fiscal = data["I1"]
        self.total_impostos = float(data["N"])
        self.total = float(data["O"])
        self.hash_suffix = data["Q"]
        self.cert_numero = int(data["R"])

        if "S" in data:
            self.info_adicional = data["S"]
        else:
            self.info_adicional = None
        
class InvoiceQRReader():
    def __init__(self):
        self.qr_codes :dict[str,InvoiceQRCode] = {}
        self.counter = 0
        self.qreader = QReader()

    def parse_qr_codes(self, img : np.ndarray) -> list[QRCode]:
        # Search for QR codes
        bboxes = self.qreader.detect(image=img)
        #decodedObjects = pyzbar.decode(img)

        list_qr_codes : list[QRCode] = []

        for i,bbox in enumerate(bboxes):
            text = self.qreader.decode(image=img, bbox=bbox)
            if text == None:
                continue
            curr_qr_code = QRCode(text, bbox, (240,0,0) ,str(i), img)
            list_qr_codes.append(curr_qr_code)

        return list_qr_codes
    
    def add_invoice_from_file(self, filename : str) -> list[QRCode]:

        if filename.endswith(".pdf"):
            list_qr_codes : list[QRCode] = []
            images = convert_from_path(filename)
            for image in images:
                list_qr_codes.extend(self.add_invoice(np.array(image)))

            return list_qr_codes

        elif filename.endswith(".jpg") or filename.endswith(".png"):
            img = cv2.imread(filename)

            if img is None:
                LOGGER.error(f"Failed to read image {filename}")
                return []
            return self.add_invoice(img)

        else:
            LOGGER.error(f"Unsupported file format {filename}")
            return []

    def add_invoice(self, img : np.ndarray) -> list[QRCode]:
        qr_list : list[QRCode] = self.parse_qr_codes(img)
        self.counter+=1
        if len(qr_list) == 0:
            cv2.imwrite(f"no_qr_{self.counter}.png", img)
            LOGGER.warning("No QR code found")
            return []

        elif len(qr_list) > 1:
            LOGGER.warning("Found more than one QR code")
            return qr_list
        
        qr_code = qr_list[0]
        
        invoice = None
        try:
            invoice = InvoiceQRCode(qr_code)
        except ValueError as e:
            LOGGER.warning("Failed to parse QR code")
            return [qr_code]
        
        qr_code.bbox_color = (0,240,0)
        
        if invoice.documento_numero in self.qr_codes:
            LOGGER.debug(f"Invoice QR code with ID {invoice.documento_numero} already exists")
            existent_qr = self.qr_codes[invoice.documento_numero]

            if existent_qr.qr_info.data != qr_code.data:
                LOGGER.warning(f"QR code with ID {invoice.documento_numero} already exists but has different data, replacing it...")
                self.qr_codes[invoice.documento_numero] = invoice
        else:
            self.qr_codes[invoice.documento_numero] = invoice
        
        return [qr_code]

    @staticmethod
    def draw_qr_info(qr_codes : list[QRCode], img : np.ndarray) -> np.ndarray:

        # Copy image

        for qr_code in qr_codes:
            pt1 = (qr_code.bbox[0],qr_code.bbox[1])
            pt2 = (qr_code.bbox[2],qr_code.bbox[3])
            # Draw bounding box
            cv2.rectangle(img, pt1, pt2, color=(255, 0, 0), thickness=2)
            # Draw title
            cv2.putText(img, qr_code.title, pt1, cv2.FONT_HERSHEY_SIMPLEX, 2, (255, 0, 0), 3)

        scale_percent = 20 # percent of original size
        width = int(img.shape[1] * scale_percent / 100)
        height = int(img.shape[0] * scale_percent / 100)
        dim = (width, height)
    
        # resize image
        resized = cv2.resize(img, dim, interpolation = cv2.INTER_AREA)
        return resized

    def save_csv_data(self,filename : str) -> None:

        if len(self.qr_codes) == 0:
            LOGGER.warning("No QR codes to save")
            return
        # Create directory if it doesn't exist
        if not os.path.exists(os.path.dirname(filename)):
            os.makedirs(os.path.dirname(filename))
        with open(filename, 'w', newline='') as csvfile:
            fieldnames = list(map(lambda field: field.name,fields(InvoiceQRCode)))
            fieldnames.remove("qr_info")
            writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

            writer.writeheader()
            for invoice in self.qr_codes.values():
                values = {field.name: getattr(invoice, field.name) for field in fields(InvoiceQRCode)}
                del values["qr_info"]
                writer.writerow(values)

    def save_images(self, path : str) -> None:
        # Create directory if it doesn't exist
        if not os.path.exists(path):
            os.makedirs(path)

        for invoice in self.qr_codes.values():
            img = invoice.qr_info.img
            self.draw_qr_info([invoice.qr_info], img)
            path_saved = os.path.join(path, f"{invoice.atcud}.png").replace(" ","")
            LOGGER.info("Saving image to " + path_saved)
            cv2.imwrite(path_saved, img)

    def found_missing_document_to_sum(self, value_sum : int) -> list[list[InvoiceQRCode]]:
        invoices_list : list[InvoiceQRCode] = list(self.qr_codes.values())
        invoices_price_int_list : list[int] = [int(invoice.total*100) for invoice in invoices_list]
        
        missing_invoices_list : list[list[InvoiceQRCode]] = []
        for solution in subsetsum.solutions(invoices_price_int_list, value_sum):
            # Check if the combination of invoices is valid
            invoices : list[InvoiceQRCode] = [invoices_list[price_idx] for price_idx in solution]
            missing_invoices_list.append(invoices)

        return missing_invoices_list
            
            

    def capture_from_video(self) -> None:
        vid = cv2.VideoCapture(0)
        vid.set(cv2.CAP_PROP_BUFFERSIZE, 1)
        while(True):

            # Capture the video frame
            # by frame

            ret, frame = vid.read()   
            qr_codes = self.add_invoice(frame)
            self.draw_qr_info(qr_codes, frame)

            # Display the resulting frame
            cv2.imshow('frame', frame)

            # the 'q' button is set as the
            # quitting button you may use any
            # desired button of your choice
            if cv2.waitKey(1) & 0xFF == ord('q'):
                break


def main():
    # Define a logger with the format [%(asctime)s] %(levelname)s: %(message)s
    logging.basicConfig(format='[%(asctime)s] %(levelname)s: %(message)s', level=logging.DEBUG)

    parser = argparse.ArgumentParser()
    parser.add_argument('--folder', help='File to process')
    parser.add_argument('--file', help='File to process')
    parser.add_argument('--load_from_camera',action='store_true', help='File to process')
    parser.add_argument('--debug', action='store_true', help='Enable debug mode')
    parser.add_argument('--output_csv', help='Output file', nargs="?", default="output/invoices.csv", const="output/invoices.csv")
    parser.add_argument('--output_images', help='Output directory for images',nargs="?",  default="output", const="output")
    args = parser.parse_args()

    invoice_reader = InvoiceQRReader()
    # Check if file argument was passed

    if args.file:
        invoice_reader.add_invoice_from_file(args.file)

    if args.folder:
        for file in os.listdir(args.folder):
            invoice_reader.add_invoice_from_file(os.path.join(args.folder, file))

    if args.load_from_camera:
        invoice_reader.capture_from_video()

    invoice_reader.save_csv_data(args.output_csv)

    if args.debug:
        invoice_reader.save_images(args.output_images)

if __name__ == "__main__":
    main()
