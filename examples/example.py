from lana import Matrix

mat = Matrix.zeros((3,3))
mat.print()
print(f"shape: {mat.shape}, type: {type(mat.shape)}")

mat = Matrix.matrix([[1,2,3],[4,5,6]])
mat.print()
print(mat.shape, type(mat))
for rows in mat.data:
    print(rows)
