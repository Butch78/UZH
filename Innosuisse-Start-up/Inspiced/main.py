# Create a standard FastAPI app

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from chef_transformer import Chef

app = FastAPI()


@app.get("/")
async def hello():
    return {"message": "Hello World"}


@app.get("/generate")
async def root(items: list[str]):
    chef = Chef()
    output = ""
    generated = chef.generation_function(items)
    for text in generated:
        sections = text.split("\n")
        for section in sections:
            section = section.strip()
            if section.startswith("title:"):
                section = section.replace("title:", "")
                headline = "TITLE"
            elif section.startswith("ingredients:"):
                section = section.replace("ingredients:", "")
                headline = "INGREDIENTS"
            elif section.startswith("directions:"):
                section = section.replace("directions:", "")
                headline = "DIRECTIONS"

            if headline == "TITLE":
                print(f"[{headline}]: {section.strip().capitalize()}")
                output.join(f"[{headline}]: {section.strip().capitalize()}")
            else:
                section_info = [
                    f"  - {i+1}: {info.strip().capitalize()}"
                    for i, info in enumerate(section.split("--"))
                ]
                print(f"[{headline}]:")
                print("\n".join(section_info))
                output.join(f"[{headline}]: {section_info}")

        print("-" * 130)
        output.join("-" * 130)
    return output
