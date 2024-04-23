import base
from ch3 import EXPECTED_3
from ch2b import NOT_EXPECTED_2b

EXPECTED_4 = EXPECTED_3 + [
    # ch4_mmap0
    "Test 04_1 OK3811152764531961704562569816539036366413242653332365631741!",

    # ch4_mmap3
    "Test 04_4 test OK3811152764531961704562569816539036366413242653332365631741!",

    # ch4_unmap
    "Test 04_5 ummap OK3811152764531961704562569816539036366413242653332365631741!",

    # ch4_unmap2
    "Test 04_6 ummap2 OK3811152764531961704562569816539036366413242653332365631741!",
]

NOT_EXPECTED_4 = NOT_EXPECTED_2b + [
    # ch4b_sbrk
    "Test sbrk failed!",

    # ch4_mmap1
    "Should cause error, Test 04_2 fail!",

    # ch4_mmap2
    "Should cause error, Test 04_3 fail!",
]

if __name__ == "__main__":
    base.test(EXPECTED_4, NOT_EXPECTED_4)