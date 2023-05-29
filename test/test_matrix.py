from lana import Matrix
import numpy as np
import unittest

class TestMatrix(unittest.TestCase):

    def test_zeros(self):
        lana_mat = Matrix.zeros((3,3))
        np_mat = np.zeros((3,3))
        lana_obj = lana_mat.data[0]
        lana_shape = lana_mat.shape
        np_obj = np_mat[0].tolist()
        np_shape = np_mat.shape
        # test
        self.assertEqual(lana_obj, np_obj)
        self.assertEqual(lana_shape, np_shape)
