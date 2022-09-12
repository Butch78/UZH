from datetime import datetime
from bank.card import CreditCard, DebitCard


class PaymentProcessor:
    def __init__(self, api_key: str) -> None:
        self.api_key = api_key

    def _check_api_key(self) -> bool:
        return self.api_key == "6cfb67f3-6281-4031-b893-ea85db0dce20"

    def charge(self, card: CreditCard | DebitCard, amount: float) -> None:
        if not card.validate_card():
            raise ValueError("Invalid card")
        if not self._check_api_key():
            raise ValueError(f"Invalid API key: {self.api_key}")
        if not card.charge(amount):
            raise ValueError("Insufficient funds")

        print(f"Charging card number {card.card_number} for ${amount/100:.2f}")
