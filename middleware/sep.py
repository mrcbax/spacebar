def shatter(jsonIn):
    ID = jsonIn[:jsonIn.find(":")]
    a = jsonIn[jsonIn.find(":")+1:]
barcode = a[:jsonIn.find(":")]
    b = jsonIn[jsonIn.find(":")+1:]

bar_name = b[:jsonIn.find(":")]
    c = jsonIn[jsonIn.find(":")+1:]

desc = c[:jsonIn.find(":")]

