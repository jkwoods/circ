import re
import sys
import random

def main():
  alphabet = [0,1,2,3,4,5,6,7,8,9,10,42]
  patternLen = 3
  docLen = 10
  rOffset = docLen - patternLen

  fpString = "["+("0,"*(rOffset-1))+"0]"


  fieldScript = """
    def main(public field[{patternLen}] P, private field[{docLen}] X, public field[{patternLen}] R) -> field:

    for field i in 0..{patternLen} do
      R[i] = if P[i] == 42 then 0 else R[i] fi
    endfor

    field fpr = 0
    for field i in 0..{patternLen} do
       field temp = P[i]*R[i]
       fpr = fpr + temp
    endfor

    field[7] fp = {fp}

    for field j in 0..{rLen} do
      for field i in 0..{patternLen} do
        field temp = X[j+i]*R[i]
        fp[j] = fp[j] + temp
      endfor
    endfor

    field z = 1
    for field i in 0..{rLen} do
      fp[i] = fp[i] - fpr
      z = z * fp[i]
    endfor

    return z""".format(patternLen=patternLen,docLen=docLen,rLen = rOffset, fp = fpString)

  filename = "KR_"+str(patternLen)+"_"+str(docLen)
  with open(filename+".zok", 'w') as f:
    f.write(fieldScript)
    
  with open(filename+".zok.in",'w') as f:
    for i in range(patternLen): 
      f.write("P."+str(i)+" "+str(random.choice(alphabet))+"\n")
    for i in range(docLen): 
      f.write("X."+str(i)+" "+str(random.choice(alphabet[:-1]))+"\n")
    for i in range(patternLen): 
      f.write("R."+str(i)+" "+str(random.choice(alphabet[:-1]))+"\n")
      
  with open(filename+".sh", 'w') as fw:
    fw.write("\necho -en \"\n"+str(patternLen)+"_"+str(docLen)+"\n\" >> timeList\n")
    fw.write("{ time ./target/release/examples/circ --inputs "+filename+".zok.in "+filename+".zok r1cs --action spartan > /dev/null ; } 2>> timeList")
  print("Done writing")


if __name__ == "__main__":
    main()


