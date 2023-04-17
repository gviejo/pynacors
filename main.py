# -*- coding: utf-8 -*-
# @Author: gviejo
# @Date:   2023-04-16 19:00:21
# @Last Modified by:   gviejo
# @Last Modified time: 2023-04-16 22:08:01
import pynakit as nak
import numpy as np
from matplotlib.pyplot import *
from numba import jit
from time import time

def restrict(time_array, data_array, starts, ends):
    """
    Jitted version of restrict

    Parameters
    ----------
    time_array : numpy.ndarray
        Description
    data_array : numpy.ndarray
        Description
    starts : numpy.ndarray
        Description
    ends : numpy.ndarray
        Description

    Returns
    -------
    TYPE
        Description
    """
    n = len(time_array)
    m = len(starts)
    ix = np.zeros(n, dtype=np.bool_)

    k = 0
    t = 0

    while ends[k] < time_array[t]:
        k += 1

    while k < m:
        # Outside
        while t < n:
            if time_array[t] >= starts[k]:
                # ix[t] = True
                # t += 1
                break
            t += 1

        # Inside
        while t < n:
            if time_array[t] > ends[k]:
                k += 1
                break
            else:
                ix[t] = True
            t += 1

        if k == m:
            break
        if t == n:
            break

    new_time_array = time_array[ix]
    new_data_array = data_array[ix]
    return (new_time_array, new_data_array)


@jit(nopython=True)
def jitrestrict(time_array, data_array, starts, ends):
    """
    Jitted version of restrict

    Parameters
    ----------
    time_array : numpy.ndarray
        Description
    data_array : numpy.ndarray
        Description
    starts : numpy.ndarray
        Description
    ends : numpy.ndarray
        Description

    Returns
    -------
    TYPE
        Description
    """
    n = len(time_array)
    m = len(starts)
    ix = np.zeros(n, dtype=np.bool_)

    k = 0
    t = 0

    while ends[k] < time_array[t]:
        k += 1

    while k < m:
        # Outside
        while t < n:
            if time_array[t] >= starts[k]:
                # ix[t] = True
                # t += 1
                break
            t += 1

        # Inside
        while t < n:
            if time_array[t] > ends[k]:
                k += 1
                break
            else:
                ix[t] = True
            t += 1

        if k == m:
            break
        if t == n:
            break

    new_time_array = time_array[ix]
    new_data_array = data_array[ix]
    return (new_time_array, new_data_array)


# tpython = []
tnumba = []
trust = []
# for n in range(100, 10000000, 100000):
for n in [10000]:
	print(n)
	# n = 100000
	starts = np.sort(np.random.uniform(0, 1000, n))
	ends = starts + np.random.uniform(1, 10, n)
	t = np.sort(np.random.uniform(0, 1000, n*2))
	d = np.random.rand(n*2)

	# t1=time()
	# a = restrict(t, d, starts, ends)
	# tpython = time() - t1


	# jitrestrict(t, d, starts, ends)
	t1=time()
	a = jitrestrict(t, d, starts, ends)
	tnumba.append(time() - t1)

	t1=time()
	a = nak.restrict(t, d, starts, ends)
	trust.append(time() - t1)


figure()
plot(tnumba[1:], label = "numba")
plot(trust[1:], label = "rust")
legend()
show()