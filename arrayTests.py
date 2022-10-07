import random
import sys
def main():
  n = int(sys.argv[1])
  var_dec  = ["private field X"+str(i) for i in range(n)]
  mults = ["\tX"+str(i)+" = X"+str(i)+"*X"+str(i-1) for i in range(1,n)] 
  array_script = "def main("+",".join(var_dec)+") -> field:\n"+"\n".join(mults)+"\n\treturn X"+str(n-1)
  
  in_script = "\n".join(["X"+str(i)+" "+str(i+1) for i in range(n)])
  in_array_script = "\n".join(["X."+str(i)+" "+str(i+1) for i in range(n)])
  with open("no_array.zok", 'w') as f:
     f.write(array_script)
  with open("no_array.in", 'w') as f: 
     f. write(in_script)
  with open("array_test.in", 'w') as f: 
     f. write(in_array_script)
  print("Done writing")


if __name__ == "__main__":
    main()

