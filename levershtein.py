import numpy as np

def levenshtein(a, b):
    a, b = a.lower(), b.lower()
    d = np.zeros((len(a), len(b)))
    d[:, 0] = np.arange(1, len(a) + 1)
    d[0, :] = np.arange(1, len(b) + 1)
    
    for i, aa in enumerate(a):
        for j, bb in enumerate(b):
            sub_cost = 0 if aa == bb else 1

            d[i, j] = min([
                d[i-1, j] + 1,
                d[i, j-1] + 1,
                d[i-1, j-1] + sub_cost
            ])
    print(d)
    return d[-1, -1]

print(levenshtein("asdf", "asdf"))