import logging
from invoice_parser.qr_invoice import InvoiceQR
from invoice_parser.qr_engine import QRDecoder
from typing import Optional
import numpy as np


class InvoiceQRParser:

    def __init__(self, qr_decoder : QRDecoder):
        self.qr_decoder = qr_decoder

    @staticmethod
    def parse_qr_decoded(string_data: str) -> InvoiceQR:
        """
        Parses a QR code string into an InvoiceQR object.

        Args:
            string_data (str): The string data of the QR code.

        Raises:
            ValueError: If the QR code string is invalid.

        Returns:
            InvoiceQR: The parsed QR code.
        """

        try:
            key_value_list = string_data.split("*")

            json_dict : dict[str,str] = {}
            for token in key_value_list:
                key_value_pair = token.split(":")
                if len(key_value_pair) != 2:
                    raise ValueError("Invalid QR code format. Please use the format specified by the Portuguese Tax Authority.")
                json_dict[key_value_pair[0]] = key_value_pair[1]
        except:
            raise ValueError("Invalid QR code format. Please use the format specified by the Portuguese Tax Authority.")


        return InvoiceQR.model_validate(json_dict)
    
    def parse_qr_invoice_img(self, img : np.ndarray) -> list[InvoiceQR]:
        """
        Parses a QR code from an image into an InvoiceQR object.

        Args:
            img (list[np.ndarray]): The image containing the QR code.

        Returns:
            list[InvoiceQR]: The parsed QR code.
        """
        data_list = self.qr_decoder.get_decoded_from_raw_img(img)

        result : list[InvoiceQR] = []
        for data in data_list:
            try:
                result.append(self.parse_qr_decoded(data))
            except:
                logging.warning(f"Failed to parse QR code {data}")
        
        return result
    
    def parse_from_file(self, filename : str) -> list[InvoiceQR]:
        """
        Parses a QR code from a file into an InvoiceQR object.

        Args:
            filename (str): The file containing the QR code.

        Returns:
            list[InvoiceQR]: The parsed QR code.
        """
        decoded_list = self.qr_decoder.get_decoded_from_local_file(filename)
        result : list[InvoiceQR] = []

        for data in decoded_list:
            try:
                result.append(self.parse_qr_decoded(data))
            except:
                logging.warning(f"Failed to parse QR code {data}")

        return [self.parse_qr_decoded(data) for data in decoded_list]