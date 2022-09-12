from typing import Protocol

from bank.card import CreditCard, DebitCard
from bank.order import Order


class PaymentProcessor(Protocol):
    def charge(self, card: CreditCard | DebitCard, amount: float) -> None:
        """Charge the card for the amount"""


def pay_order(
    order: Order, payment_processor: PaymentProcessor, card: CreditCard | DebitCard
) -> None:
    """Pay for the order using the provided payment processor"""
    if order.total == 0.0:
        raise ValueError("Order is empty")
    payment_processor.charge(card, order.total)
    order.pay()
