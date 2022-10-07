import random
import sys
def main():
  n = int(sys.argv[1])
  var_dec  = ["private field X"+str(i) for i in range(n)]
  mults = ["\tX"+str(i)+" = X"+str(i)+"*X"+str(i-1) for i in range(1,n)] 
  c_array = ["__attribute__((private(0))) in X"+str(i) for i in range(n)]
  array_script = "def main("+",".join(var_dec)+") -> field:\n"+"\n".join(mults)+"\n\treturn X"+str(n-1)
  
  in_script = "\n".join(["X"+str(i)+" "+str(i+1) for i in range(n)])
  in_array_script = "\n".join(["X."+str(i)+" "+str(i+1) for i in range(n)])
  c_script = """
  #include <stdint.h>
  int main("""+",".join(c_array) + ") {\n"+";\n".join(mults) + ";\nreturn X"+str(n-1)+";}"
  with open("no_array.zok", 'w') as f: 
     f.write(array_script)
  with open("no_array.in", 'w') as f: 
     f. write(in_script)
  with open("array_test.in", 'w') as f: 
     f. write(in_array_script)
  with open("array_c_test.in", 'w') as f: 
     f. write(c_script)
  print("Done writing")


if __name__ == "__main__":
    main()

