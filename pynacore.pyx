import numpy as np
cimport numpy as np
np.import_array()


def restrict(   np.ndarray time_array, 
                np.ndarray data_array, 
                np.ndarray starts, 
                np.ndarray ends):
    cdef int n = len(time_array)
    cdef int m = len(starts)
    cdef np.ndarray ix
    ix = np.zeros(n, dtype=np.bool_)

    cdef int k = 0
    cdef int t = 0

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