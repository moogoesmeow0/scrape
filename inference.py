import torch
from transformers import AutoTokenizer, AutoModelForSequenceClassification
import numpy as np

MODEL_PATH = "clickbait_regressor"  

tokenizer = AutoTokenizer.from_pretrained(MODEL_PATH)
model = AutoModelForSequenceClassification.from_pretrained(MODEL_PATH)

model.eval()

def predict_clickbaitiness(headlines):
    if isinstance(headlines, str):
        headlines = [headlines]
    
    inputs = tokenizer(headlines, return_tensors="pt", padding=True, truncation=True)

    with torch.no_grad():
        outputs = model(**inputs)
        logits = outputs.logits.squeeze()

    # Ensure outputs are numpy array
    scores = logits.cpu().numpy()
    
    # If only one prediction, wrap in array
    if scores.ndim == 0:
        scores = np.array([scores])

    #denormalize
    return np.clip(scores * 100, 1, 100).tolist()

if __name__ == "__main__":
    example_headlines = [
        "This simple trick will save you thousands",
        "Apple announces Q4 earnings and new product line",
        "Doctors hate him: see how he lost 50 pounds in 2 weeks",
        "UN votes on emergency climate resolution"
    ]

    predictions = predict_clickbaitiness(example_headlines)
    for headline, score in zip(example_headlines, predictions):
        print(f"[{score:.2f}/100] {headline}")
