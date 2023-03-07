import streamlit as st
import pandas as pd
import numpy as np

from chef_transformer import Chef

st.title("Inspiced")

import streamlit as st

with st.form("my_form"):
    input_text = st.text_input("Enter text")

    submitted = st.form_submit_button("Submit")
    if submitted:
        st.write("Submitted!", input_text)

    chef = Chef()
    generated_recipe = chef.generation_function(input_text)

if submitted:
    for text in generated_recipe:
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
                st.text(f"[{headline}]: {section.strip().capitalize()}")
            else:
                section_info = [
                    f"  - {i+1}: {info.strip().capitalize()}"
                    for i, info in enumerate(section.split("--"))
                ]
                print(f"[{headline}]:")
                st.text(f"[{headline}]:")
                print("\n".join(section_info))
                st.text("\n".join(section_info))

        print("-" * 130)
