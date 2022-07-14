import sys
import time
import os
import datagears
    

def model_hook(k):
    for key, value in k.items():
        print ("KEY: ", key)
        print ("VALUE: ", value)

    k.update({
        "response": "yes",
        "error": "",
    })

    return k


@datagears.pymodel
def run(dg: "datagears"):
    """"""
    iter = dg.iter("my-iter")
    
    dg.info("reading 10 records from the stream")
    window = iter.read(10)
    result = window.map(lambda record: model_hook(record))
    
    dg.info("writing result")
    dg.write("my-stream2", result)
    
    
    
