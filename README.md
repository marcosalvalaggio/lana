## Lana 🧶

[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#work-in-progress) 

<em><span style="font-weight:bold;color:blue;">L</span>inear</em> <span style="font-weight:bold;color:red;">a</span>lgebra for <em><span style="font-weight:bold;color:green;">n</span>octurnal</em> and <em><span style="font-weight:bold;color:orange;">a</span>dventurous</em> data scientists.


In the example above, lines starting with `-` will be displayed in red, and lines starting with `+` will be displayed in green.

Note that the above example uses the `diff` language for syntax highlighting, which is widely supported. You can also use other supported languages for different color styles. Make sure to wrap your text within the appropriate syntax highlighting language.

When you commit and push your changes to GitHub, the colors specified using syntax highlighting will be rendered correctly on the GitHub web page.


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

