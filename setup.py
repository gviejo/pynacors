# -*- coding: utf-8 -*-
# @Author: gviejo
# @Date:   2023-04-24 14:24:13
# @Last Modified by:   gviejo
# @Last Modified time: 2023-04-24 14:35:00
from distutils.core import setup
from Cython.Build import cythonize

setup(ext_modules = cythonize('pynacore.pyx'))