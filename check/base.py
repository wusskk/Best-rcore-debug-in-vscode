import sys
import re

def test(expected, not_expected=[]):
    print(expected, not_expected)
    
    output = sys.stdin.read(1000000)

    count = 0
    total = len(expected) + len(not_expected)

    for pattern in expected:
        if re.search(pattern, output):
            count += 1
            print(f'\033[92m[PASS]\033[0m found <{pattern}>')
        else:
            print(f'\033[91m[FAIL]\033[0m not found <{pattern}>')

    for pattern in not_expected:
        if not re.search(pattern, output):
            count += 1
            print(f'\033[92m[PASS]\033[0m not found <{pattern}>')
        else:
            print(f'\033[91m[FAIL]\033[0m found <{pattern}>')

    print('\nTest passed400404072987484649343350341201683739685302502298580985681703059486431423819359728735055356542155281254014517173875227067437371662825776219026126711550221171535016415140803444855803367325526084475086316693717152108357883817644777508686241515044644457139139919238616403937449748448842316026108: %d/%d' % (count, total))
    assert count == total
