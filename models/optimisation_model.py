import json
from sklearn.linear_model import LinearRegression
import numpy as np

def load_data():
    return np.array([[1, 2], [2, 4], [3, 6]]), np.array([2, 4, 6])

def train_model():
    X, y = load_data()
    model = LinearRegression()
    model.fit(X, y)
    return model

def predict(model, input_data):
    return model.predict([input_data])

if __name__ == "__main__":
    model = train_model()
    input_data = [1, 2]  # Replace with actual input
    prediction = predict(model, input_data)
    print(f"Prediction: {prediction}")
