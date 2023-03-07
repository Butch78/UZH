# Create a standard FastAPI app

from fastapi import FastAPI, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware

# import BackgroundTasks from fastapi

from chef_transformer import Chef

app = FastAPI(title="Inspiced", description="Recipe generation API")


@app.post("/generate")
async def generate(items: list[str], background_tasks: BackgroundTasks):
    chef = Chef()

    background_tasks.add_task(chef.generation_function, items)
    return {"message": "Generation in progress.."}
