## Lana 🧶

[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#work-in-progress) 

**L***inear* **A**lgebra for **n***octurnal* *and* **a**dventurous *data scientists.*

## Install 

```console
pip install lana
```

## Example 

```python
from lana import Matrix, inject

zeros = Matrix.zeros((3,3))
print(zeros)
print(f"shape: {zeros.shape}, type: {type(zeros)}")
print(zeros.to_list()[0], zeros.to_list()[0][0])

mat = Matrix.matrix([[1,2,3],[4,5,6]])
print(mat)
print(f"shape: {mat.shape}, type: {type(mat)}")
for rows in mat.to_list():
    print(rows, type(rows))

submat = Matrix.matrix(inject(mat.to_list()[0]))
print(submat)
print(submat.shape, type(submat))
```

## Manual Build

To build a package wheel, run the following command:

```console
pip install maturin
git clone https://github.com/marcosalvalaggio/lana.git
cd lana
maturin build --sdist
maturin develop
```