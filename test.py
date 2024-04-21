import pickle
import os

ROOT_DIR = os.path.dirname(__file__)
DATA_DIR = f"{ROOT_DIR}/data/"


test1 = (1, 2, 3, 4, (5, 6, 7), 'Test', ('This is just a test.', [2, 4, 6, 8]), 'One', 'Two', 'Three')
test2 = (1, 2.0, 3, 4.0, (5, 6, 7), 'Test', ('This is just a test.', [2, 4, 6, 8]), 'One', 'Two', 'Three')


def write_pickle_file(test_name, data): 
  with open(f"{DATA_DIR}{test_name}", "wb") as f:
    pickle.dump(data, f)

def read_pickle_file(test_name):
  with open(f"{DATA_DIR}{test_name}", "rb") as f:
    a = pickle.load(f)
    print(a)


write_pickle_file("test1", test1)
write_pickle_file("test2", test2)
read_pickle_file("test1")
read_pickle_file("test2")
