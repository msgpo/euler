import random
from heapLists import *

# bozo3 sort a given sequence
def bozo3Sort(sequence, shuffles):
    # check if sorted
    if isSorted(sequence):
    #   if so, quit
        return shuffles
    # randomly select 3 items
    selectI = random.sample(range(0, 11), 3)
    shuffleI = selectI
    # randomly shuffle them
    random.shuffle(shuffleI)
    # increment shuffles & recurse
    return bozo3Sort(sequence, shuffles + 1)


# check if a sequence is sorted
def isSorted(sequence):
    memory = 0
    for num in sequence:
        if num < memory:
            return False
        memory = num
    return True

def solver():
    generator = heapLists()
    seqs = generator.generateLists()
    counts = {}
    for seq in seqs:
        print("Sequence: {}".format(seq))
        count = bozo3Sort(seq, 0)
        print("Sorted after {} shuffles".format(count))
        counts[count] = counts.get(count, 0) + 1
    total = 0
    for count, freq in counts.items():
        total = total + count * freq
    print ("Total shuffles {} to sort {} permutations".format(total, len(seqs)))
    print ("Average shuffles per permutation is {}".format(total / len(seqs)))

if __name__ == "__main__":
    solver()
