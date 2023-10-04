import logging
from invoice_parser.qr_invoice import InvoiceQRParser, InvoiceQR
from invoice_parser.qr_engine import QRDecoder
from invoice_parser.excel_manager import ExcelManager
import cv2 as cv
import os
import argparse

PATH_TO_PARSED_INVOICES = "parsed_invoices"
EXCEL_FILE = os.path.join(PATH_TO_PARSED_INVOICES,"parsed_invoices.xlsx")

class QRInvoiceDumper:

    def __init__(self):
        """
        Initializes a QRInvoiceDumper object.
        """
        self.qr_engine = QRDecoder()
        self.qr_invoice_parser = InvoiceQRParser(self.qr_engine)
        self.excel_manager = ExcelManager(EXCEL_FILE)

        # Create dir if it doesn't exist
        if not os.path.exists(PATH_TO_PARSED_INVOICES):
            os.makedirs(PATH_TO_PARSED_INVOICES)

    def __save_qr_invoice(self, qr_invoice : InvoiceQR):
        """
        Saves a QR invoice to a json file.

        Args:
            qr_invoice (InvoiceQR): The QR invoice to save.
        """
        path_saved = os.path.join(PATH_TO_PARSED_INVOICES, f"{qr_invoice.atcud}.json")
        self.excel_manager.add_invoice(qr_invoice)
        with open(path_saved, "w") as f:
            data = qr_invoice.model_dump_json(indent=4)
            f.write(data)

    def dump_from_local_file(self, filename : str):
        """
        Dumps QR invoices from local files.

        Args:
            filenames (str): The filename to read.
        """
        qr_invoices = self.qr_invoice_parser.parse_from_file(filename)
        for qr_invoice in qr_invoices:
            self.__save_qr_invoice(qr_invoice)
        
        self.excel_manager.save_file()

    def dump_from_local_dir(self, dir : str):
        """
        Dumps QR invoices from local files.

        Args:
            dir (str): The directory to read.
        """
        for filename in os.listdir(dir):
            self.dump_from_local_file(os.path.join(dir, filename))

        self.excel_manager.save_file()

    def dump_from_camera(self):
        """
        Dumps QR invoices from the camera.
        """
        vid = cv.VideoCapture(0)

        while(True):
                
            # Capture the video frame
            # by frame
            ret, frame = vid.read()

            cv.imshow('frame', frame)
            
            qr_invoice = self.qr_invoice_parser.parse_qr_invoice_img(frame)
            if qr_invoice is not None:
                self.__save_qr_invoice(qr_invoice)

            # the 'q' button is set as the
            # quitting button you may use any
            # desired button of your choice
            if cv.waitKey(1) & 0xFF == ord('q'):
                break

def main():
    logging.basicConfig(level=logging.INFO) 
    parser = argparse.ArgumentParser(description='Dumps QR invoices from local files')
    parser.add_argument('-f','--file', type=str, help='The file to read.')
    parser.add_argument('-d','--directory', type=str, help='The file to read.')
    args = parser.parse_args()
    

    if args.file is not None:
        qr_invoice_dumper = QRInvoiceDumper()
        qr_invoice_dumper.dump_from_local_file(args.file)
    elif args.directory is not None:
        qr_invoice_dumper = QRInvoiceDumper()
        qr_invoice_dumper.dump_from_local_dir(args.directory)

    

if __name__ == "__main__":
    main()