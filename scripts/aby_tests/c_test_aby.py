#!/usr/bin/env python

from utils import run_tests
from test_suite import *

if __name__ == "__main__":
    # tests = arithmetic_tests + \
    #     mod_tests + \
    #     arithmetic_boolean_tests + \
    #     nary_arithmetic_tests + \
    #     bitwise_tests + \
    #     boolean_tests + \
    #     nary_boolean_tests + \
    #     const_arith_tests + \
    #     const_bool_tests + \
    #     ite_tests + \
    #     array_tests + \
    #     c_array_tests + \
    #     div_tests + \
    #     shift_tests

    # tests = ilp_benchmark_tests
    # tests = kmeans_tests + div_tests
    # tests = kmeans_tests
    # tests = arithmetic_tests
    # tests = div_tests
        
    # TODO: add support for return value - int promotion
    # unsigned_arithmetic_tests + \

    tests = biomatch_tests
    run_tests('c', tests)
