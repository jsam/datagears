import sys
import time
import os
from datagears import DataGears, pygear
    

def model_hook(k):
    for key, value in k.items():
        print ("KEY: ", key)
        print ("VALUE: ", value)

    k.update({
        "response": "yes",
        "error": "",
    })

    return k


@pygear
def run(dg: "DataGears"):
    """"""
    iter = dg.iter("my-iter").read(10)  # Stateful iterators: Get 10 items from the last known state of `my-iter`
    iter = dg.iter().last(10) # Stateless iterator: Get last 10 items
    iter = dg.iter().last()  # Stateless iterator: Get last item
    log = dg.logging()  # Add a logger for the current execution.
    exec_id = dg.execution_id()  # Get the execution identifier for this run of the gear.
    
    try:
        result = df.map(lambda record: model_hook(record))
    except Exception as e:
        dg.error(e)
    
    
    log.info("reading 10 records from the stream")
    df = iter.read(10)
    
    fruits = df.sort("fruits").select(["fruits"])
    dg.info("writing result")
    dg.write("only-fruits", fruits)
    
    
    
    
    
    
