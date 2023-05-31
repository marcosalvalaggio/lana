from typing import Union, List
from .lana import Matrix

def inject(data: Union[float, int, List, List[List]]):
    if isinstance(data, float) or isinstance(data, int):
        matrix = [[float(data)]]
        return matrix
    elif isinstance(data, list):
        if all(isinstance(item, list) for item in data):
            return data 
        else: 
            matrix = [data]
            return matrix
    else: 
        raise TypeError("check the input data")

__all__ = [
    "Matrix",
    "inject",
]