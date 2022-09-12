__version__ = "0.1.0"

from dataclasses import dataclass
from datetime import datetime


@dataclass
class Card:
    def __init__(
        self,
        card_number: str,
        balance: float,
        expiry_month: int,
        expiry_year: int,
    ):
        self.card_number = card_number
        self.balance = balance
        self.expiry_month = expiry_month
        self.expiry_year = expiry_year

    @property
    def get_card_number(self) -> str:
        return self.card_number


    def get_balance(self) -> float:
        return self.balance

    def set_balance(self, balance) -> float:
        self.balance = balance
        return self.balance

    def validate_card(self) -> bool:
        return (
            luhn_checksum(self.card_number)
            and datetime(self.expiry_year, self.expiry_month, 1) > datetime.now()
        )

    def charge(self, amount: float) -> bool:
        if amount > self.balance:
            return False
        self.balance -= amount
        return True


# Credit Card Class
class CreditCard(Card):
    def __init__(
        self,
        card_number: str,
        balance: float,
        credit_limit: float,
        expiry_month: int,
        expiry_year: int,
    ):
        super().__init__(card_number, balance, expiry_month, expiry_year)
        self.credit_limit = credit_limit

    @property
    def get_credit_limit(self) -> float:
        return self.credit_limit

    def set_credit_limit(self, credit_limit: float) -> float:
        self.credit_limit = credit_limit
        return self.credit_limit

    def charge(self, amount: float) -> bool:
        if amount > self.balance + self.credit_limit:
            return False
        self.balance -= amount
        return True


# Debit Card Class
class DebitCard(Card):
    def __init__(
        self,
        card_number: str,
        balance: float,
        expiry_month: int,
        expiry_year: int,
        overdraft_limit: float = 0.0,
    ):
        super().__init__(card_number, balance, expiry_month, expiry_year)
        self.overdraft_limit = overdraft_limit

    def get_overdraft_limit(self):
        return self.overdraft_limit

    def set_overdraft_limit(self, overdraft_limit):
        self.overdraft_limit = overdraft_limit

    def charge(self, amount: float) -> bool:
        if amount > self.balance + self.overdraft_limit:
            return False
        self.balance -= amount
        return True


def luhn_checksum(card_number: str) -> bool:
    """Check if the card number is valid using the Luhn algorithm."""

    def digits_of(card_number: str):
        return [int(d) for d in card_number]

    digits = digits_of(card_number)
    odd_digits = digits[-1::-2]
    even_digits = digits[-2::-2]
    checksum = 0
    checksum += sum(odd_digits)
    for d in even_digits:
        checksum += sum(digits_of(str(d * 2)))
    return checksum % 10 == 0
