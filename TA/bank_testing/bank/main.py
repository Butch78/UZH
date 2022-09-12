import os

from dotenv import load_dotenv

from bank.card import Card, CreditCard, DebitCard
from bank.order import LineItem, Order
from bank.payment import pay_order
from bank.processor import PaymentProcessor


def read_card_info():
    card_number = input("Card number: ")
    card_type = input("Card type (credit/debit): ")
    if card_type == "credit":
        return CreditCard(card_number, 0.0, 1000.0, 1, 2023)
    elif card_type == "debit":
        return DebitCard(card_number, 0.0, 1, 2023)
    else:
        raise ValueError("Invalid card type")


def main():
    load_dotenv()

    api_key = os.getenv("API_KEY")
    if api_key is None:
        raise ValueError("API key not set")
    payment_processor = PaymentProcessor(api_key)
    # Test card number: 1249190007575069
    order = Order()
    order.line_items.append(LineItem("T-Shirt", 1, 10.0))
    order.line_items.append(LineItem("T-Shirt", 1, 10.0))

    # Read
    card = read_card_info()
    pay_order(order, payment_processor, card)


if __name__ == "__main__":
    main()
