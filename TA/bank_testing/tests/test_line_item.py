from bank.order import LineItem

def test_line_item_default() -> None:
    line_item = LineItem("T-Shirt", 1, 10.0)
    assert line_item.name == "T-Shirt"
    assert line_item.quantity == 1
    assert line_item.price == 10.0
    assert line_item.total == 10.0