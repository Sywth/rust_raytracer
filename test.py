import random, math

def ff() -> tuple[float,float,float]:
    x = random.uniform(-math.sqrt(1/3),math.sqrt(1/3))
    y = random.uniform(-math.sqrt(1/3),math.sqrt(1/3))
    z = random.uniform(-math.sqrt(1/3),math.sqrt(1/3))
    return (x,y,z)

def length(v):
    return v[0]*v[0] + v[1]*v[1] + v[2]*v[2]

l1 = [(length(r := ff()),r) for _ in range(1000000)]

print(max(l1,key=lambda x: 
          x[1][2]))
#print(length((0.546653297619822, 0.5064117815143163, 0.46130669499366517)))