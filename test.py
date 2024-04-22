import pickle
import os

ROOT_DIR = os.path.dirname(__file__)
DATA_DIR = f"{ROOT_DIR}/data/"

def write_pickle_file(test_name, data): 
  with open(f"{DATA_DIR}{test_name}", "wb") as f:
    pickle.dump(data, f)

def read_pickle_file(test_name):
  with open(f"{DATA_DIR}{test_name}", "rb") as f:
    a = pickle.load(f)
    print(a)


test1 = (1, 2.0, 3, 4.0, (5, False, 7), 'Test', ('This is just a test.', [2, 4, 6, True]), 'One', 'Two', 'Three')
write_pickle_file("test1", test1)

test2 = {
  'a': ((5, 6, 7, 'This is just a test.', [2, 4, 6, 8]), 'One', 'Two', 'Three'),
  'b': 10e11
}
write_pickle_file("test2", test2)

class Dog:
  def __init__(self, name):
    self.name = name
    self.goodboy = True

  def bark(self):
    print(f"{self.name} said: BARK!")

test3 = Dog("bob")
write_pickle_file("test3", test3)

test3.friend = Dog("tau")
write_pickle_file("test4", test3)



read_pickle_file("test1")
read_pickle_file("test2")
read_pickle_file("test3")
read_pickle_file("test4")
