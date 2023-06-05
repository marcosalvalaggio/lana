## Lana 🧶

[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#work-in-progress) 

*<span style="font-weight:bold;color:blue;">L</span>inear* <span style="font-weight:bold;color:red;">a</span>lgebra for *<span style="font-weight:bold;color:green;">n</span>octurnal* and *<span style="font-weight:bold;color:orange;">a</span>dventurous* data scientists.


## Install 

```console
pip install lana
```

## Example 

```python
from lana import Matrix

zeros = Matrix.zeros((3,3))
print(zeros)
print(f"shape: {zeros.shape}, type: {type(zeros)}")
print(zeros.to_list()[0], zeros.to_list()[0][0])

mat = Matrix.matrix([[1,2,3],[4,5,6]])
print(mat)
print(f"shape: {mat.shape}, type: {type(mat)}")
for rows in mat.to_list():
    print(rows, type(rows))
```

