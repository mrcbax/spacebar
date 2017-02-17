def charCheck(jsonIn):
    if(colCheck(jsonIn)):
        if(perCheck(jsonIn)):
            if(assCheck(jsonIn)):
                if(bksCheck(jsonIn)):
                    return True
    return False

def colCheck(jsonIn):
    semi = 0
    for i in jsonIn:
        if(i==':'):
            semi += 1
    if(semi == 4):
        return True
    else:
        return False

def perCheck(jsonIn):
  for i in range(len(jsonIn)-1):
        if(jsonIn[i] == '.'):
            if(jsonIn[i+1] == '.'):
                return False
  return True

def assCheck(jsonIn):
    for i in jsonIn:
        if(i=='*'):
            return False
    else:
        return True

def bksCheck(jsonIn):
    for i in jsonIn:
        if(i=='\\'):
            return False
    else:
        return True
