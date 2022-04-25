from dbm import dumb
import os
import random
path = "."
dumb_content = ["apple","booper","charlie","doggy","eevee","floopiedoop","hubba","icky","joooomjoom","kawoozie","lumpo","moopadoop","noofnoof","ooolalala","poopoo","quibbles","rabble","soup","tamborino","uvula","vulmo","woomba","x gon giv it 2 u","yotta","zimple"]
for group in os.walk("."):
    dirpath, dirname, files = group
    
    for dir in dirname:
        for i in range(0,0x100):
            dirname = str(i)+"filler"
            filename = os.path.join(dir,dirname)
            #os.mkdir(dirname)
            with open(filename,"w") as filler:
                filler.write(random.choice(dumb_content)*random.randrange(10,0x100))