from lana import Matrix

mat = Matrix.zeros((3,3))
print(mat)
print(f"shape: {mat.shape}, type: {type(mat)}")

for rows in mat.data:
    print(rows)


