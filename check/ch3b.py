import base
from ch2b import NOT_EXPECTED_2b

EXPECTED_3b = [
    # ch3b_yield0
    "Test write A OK10593845765509622145578366413242653332365631741!",

    # ch3b_yield1
    "Test write B OK10593845765509622145578366413242653332365631741!",

    # ch3b_yield2
    "Test write C OK10593845765509622145578366413242653332365631741!",
]

if __name__ == "__main__":
    base.test(EXPECTED_3b, NOT_EXPECTED_2b)
