import numpy as np
from qreader import QReader
from pdf2image import convert_from_path
from typing import Optional
import cv2 as cv
import logging


class QRDecoder:
    def __init__(self):
        """
        Initializes the QRDecoder.
        """
        self.qreader = QReader()

    def __detect_and_decode(self, img : np.ndarray, is_bgr = True) -> Optional[str]:
        """
        Detects and decodes a QR code from an image.

        Args:
            img (np.ndarray): The image to be decoded.

        Returns:
            Optional[str]: The decoded QR code.
        """
        data = self.qreader.detect_and_decode(image=img, is_bgr=is_bgr)
        if data is None or len(data) == 0:
            return None
        return data[0]

    def get_decoded_from_raw_img(self, img : np.ndarray) -> list[str]:
        """
        Handles raw images.

        Args:
            img (np.ndarray): The image to be decoded in RGB format.

        Returns:
            list[str]: The decoded QR code.
        """
        data = self.__detect_and_decode(img)
        if data is None:
            return []
        
        return [data]
    
    def get_decoded_from_local_file(self, filename : str) -> list[str]:
        """
        Handles pdf and image files.

        Args:
            filename (str): Either a pdf or an image file.

        Returns:
            list[str]: The decoded QR code.
        """

        logging.info(f"Reading file {filename}")
        results : list[str] = []

        if filename.endswith(".pdf"):
            pdf = convert_from_path(filename)
            for page in pdf:
                decoded = self.__detect_and_decode(np.array(page), is_bgr=False)

                if decoded is None:
                    logging.error(f"Failed to decode image {filename}")
                    continue

                results.append(decoded)

        elif filename.endswith(".jpg") or filename.endswith(".png"):
            img = cv.imread(filename)

            if img is None:
                logging.error(f"Failed to read image {filename}")
                return []

            decoded = self.__detect_and_decode(img)
            if decoded is None:
                logging.error(f"Failed to decode image {filename}")
                return []
            
            results.append(decoded)

        logging.info("A total of {} QR codes were decoded.".format(len(results)))
        return results