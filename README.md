## Lana

```console
    .-.'  `; `-._  __  _
   (_,         .-:'  `; `-._
 ,'o"(        (_,           )
(__,-'      ,'o"(            )>
   (       (__,-'            )
    `-'._.--._(             )
       |||  |||`-'._.--._.-'
                  |||  |||
```

**L***inear* **a***lgebra* *for* **n***octurnal and* **a***dventurous data scientists.*

## Install 

```console
pip install lana
```

## Example 

```python
from lana import Matrix

print("# zeros #")
mat = Matrix.zeros((3,3))
print(mat)
print(f"shape: {mat.shape}, type: {type(mat)}")

print("# matrix #")
mat = Matrix.matrix([[1,2,3],[4,5,6]])
print(mat)
print(f"shape: {mat.shape}, type: {type(mat)}")
for rows in mat.data:
    print(rows, type(rows))
```

