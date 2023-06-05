import numpy as np
import time 
from lana import Matrix

a = np.eye((4096))
b = np.eye((4096))
s = time.monotonic()
c = a + b
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")

s = time.monotonic()
d = -c
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")

s = time.monotonic()
f = a * b
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")

a = Matrix.eye((4096))
b = Matrix.eye((4096))
s = time.monotonic()
c = a + b
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")

s = time.monotonic()
d = -c
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")


s = time.monotonic()
f = a * b
e = time.monotonic()
t = e - s
print(f"{t:.2f} sec")
