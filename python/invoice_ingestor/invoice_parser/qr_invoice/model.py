from pydantic import BaseModel, Field, validator
from datetime import date, datetime
from typing import Optional
import pandas as pd

class InvoiceQR(BaseModel):

    nif_issuer: int = Field(..., alias="A")
    nif_acquirer: int = Field(..., alias="B")
    country: str = Field(..., alias="C",max_length=12)
    doc_type: str = Field(..., alias="D",max_length=2)
    doc_state: str = Field(..., alias="E",max_length=1)
    doc_date: date = Field(..., alias="F")
    unique_id: str = Field(..., alias="G",max_length=60)
    atcud: str = Field(..., alias="H",max_length=70)

    fiscal_space: str = Field(alias="I1",max_length=5)
    base_taxable_exempt_iva: Optional[float] = Field(default=None, alias="I2")
    base_taxable_reduced_iva: Optional[float] = Field(default=None, alias="I3")
    total_reduced_iva: Optional[float] = Field(default=None, alias="I4")
    base_taxable_intermediate_iva: Optional[float] = Field(default=None, alias="I5")
    total_intermediate_iva: Optional[float] = Field(default=None, alias="I6")
    base_taxable_normal_iva: Optional[float] = Field(default=None, alias="I7")
    total_normal_iva: Optional[float] = Field(default=None, alias="I8")
    
    fiscal_space_2: Optional[str] = Field(default=None, alias="J1",max_length=5)
    base_taxable_exempt_iva_2: Optional[float] = Field(default=None, alias="J2")
    base_taxable_reduced_iva_2: Optional[float] = Field(default=None, alias="J3")
    total_reduced_iva_2: Optional[float] = Field(default=None, alias="J4")
    base_taxable_intermediate_iva_2: Optional[float] = Field(default=None, alias="J5")
    total_intermediate_iva_2: Optional[float] = Field(default=None, alias="J6")
    base_taxable_normal_iva_2: Optional[float] = Field(default=None, alias="J7")
    total_normal_iva_2: Optional[float] = Field(default=None, alias="J8")

    fiscal_space_3: Optional[str] = Field(default=None, alias="K1",max_length=5)
    base_taxable_exempt_iva_3: Optional[float] = Field(default=None, alias="K2")
    base_taxable_reduced_iva_3: Optional[float] = Field(default=None, alias="K3")
    total_reduced_iva_3: Optional[float] = Field(default=None, alias="K4")
    base_taxable_intermediate_iva_3: Optional[float] = Field(default=None, alias="K5")
    total_intermediate_iva_3: Optional[float] = Field(default=None, alias="K6")
    base_taxable_normal_iva_3: Optional[float] = Field(default=None, alias="K7")
    total_normal_iva_3: Optional[float] = Field(default=None, alias="K8")

    iva_exempt: Optional[float] = Field(default=None, alias="L")
    stamp_tax: Optional[float] = Field(default=None, alias="M")
    total_tax: float = Field(..., alias="N")
    total_gross: float = Field(..., alias="O")
    withholding_tax: Optional[float] = Field(default=None, alias="P")
    hash: str = Field(..., alias="Q",max_length=4)
    certificate_number: int = Field(..., alias="R")
    additional_info: Optional[str] = Field(default=None, alias="S",max_length=65)

    @validator('doc_date', pre=True)
    def parse_date(cls, v):
        try:
            # Attempt to parse the date string as YYYYMMDD
            date_obj = datetime.strptime(v, '%Y%m%d').date()
            return date_obj
        except ValueError:
            raise ValueError("Invalid date format. Please use YYYYMMDD.")
        
    def model_dump_pandas_series(self):
        """
        Dumps the QR invoice as a pandas series.
        """
        return pd.Series(self.dict())
    
    @staticmethod
    def dump_field_names() -> list[str]:
        """
        Dumps the QR invoice field names.
        """
        schema = InvoiceQR.model_json_schema()
        return list(schema["$defs"].keys())