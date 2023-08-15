# -*- coding: utf-8 -*-
# @Author: gviejo
# @Date:   2023-04-24 14:24:13
# @Last Modified by:   gviejo
# @Last Modified time: 2023-08-14 19:39:28
from distutils.core import setup
from Cython.Build import cythonize

setup(ext_modules = cythonize('pynacors.pyx'))