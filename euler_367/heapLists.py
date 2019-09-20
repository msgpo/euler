class heapLists:
    lists = []

    def __init__(self):
        print("Made new heaplists object")

    def generateLists(self):
        init = list(range(1, 12))
        self.heapAlgorithm(len(init), init)
        return self.lists
    
    def swap(self, i, j, sequence):
        mem = sequence[i]
        sequence[i] = sequence[j]
        sequence[j] = mem
        return sequence

    def heapAlgorithm(self, k, sequence):
        if k == 1:
            print("Discovered permutation: {}".format(sequence))
            self.lists.append(sequence)
            return
        for i in range(len(sequence)):
            if (k % 2) == 0:
                sequence = self.swap(i, k - 1, sequence)
            else:
                sequence = self.swap(0, k - 1, sequence)
            self.heapAlgorithm(k - 1, sequence)