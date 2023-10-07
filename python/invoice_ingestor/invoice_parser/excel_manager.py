import logging
import pandas as pd
import os
from invoice_parser.qr_invoice.model import InvoiceQR


class ExcelManager:

    def __init__(self, excel_file_path : str):
        """
        Initializes an ExcelManager object.

        Args:
            excel_file_path (str): The path to the excel file.
        """
        self.__excel_file = excel_file_path
        self.__dataframe : list[dict] = []

        # Create dir if it doesn't exist
        if not os.path.exists(os.path.dirname(self.__excel_file)):
            os.makedirs(os.path.dirname(self.__excel_file))

        if os.path.exists(self.__excel_file):
            self.__dataframe = pd.read_excel(self.__excel_file, engine='openpyxl').to_dict('records')

    def add_invoice(self, invoice : InvoiceQR):
        """
        Adds an invoice to the excel file.

        Args:
            invoice (InvoiceQR): The invoice to add.
        """
        self.__dataframe.append(invoice.model_dump())


    def __del__(self):
        logging.info(f"Saving excel file to {self.__excel_file}")
        if len(self.__dataframe) > 0:
            pd.DataFrame(self.__dataframe).drop_duplicates().to_excel(self.__excel_file, index=False)