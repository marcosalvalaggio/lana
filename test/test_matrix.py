from lana import Matrix
import numpy as np
import unittest

class TestMatrix(unittest.TestCase):

    def test_zeros(self):
        lana_mat = Matrix.zeros((3,3))
        np_mat = np.zeros((3,3))
        lana_obj = lana_mat[0]
        lana_shape = lana_mat.shape
        np_obj = np_mat[0].tolist()
        np_shape = np_mat.shape
        # test
        self.assertEqual(lana_obj, np_obj)
        self.assertEqual(lana_shape, np_shape)

    def test_simple_operation(self):
        # lana
        a = Matrix.ones((2,3))
        b = Matrix.matrix([[2,2,2],[2,2,2]])
        c = a + b
        d = c - a
        e = -d 
        # numpy
        a_np = np.ones((2,3))
        b_np = np.array([[2,2,2],[2,2,2]])
        c_np = a_np + b_np
        d_np = c_np - a_np
        e_np = -d_np
        for i, elem in enumerate(e_np):
            np.testing.assert_array_equal(elem, e[i])

        
