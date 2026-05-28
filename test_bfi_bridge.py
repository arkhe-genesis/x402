import pytest
from unittest.mock import MagicMock, patch
from substrate_933_bfi_bridge import SPBConfig, BfiBridge, BrazilianFinancialInfrastructureBridge

def test_ontology():
    ontology = BrazilianFinancialInfrastructureBridge()
    assert "integrates the Brazilian Payment System" in ontology.statement
    assert "SPB Adapter" in ontology.components
    assert "SPI Hook" in ontology.components

def test_send_str_order():
    mock_agent = MagicMock()
    config = SPBConfig()
    bridge = BfiBridge(mock_agent, config)

    order_id = bridge.send_str_order(100.50, "12345-6", "Payment")

    assert len(order_id) == 16
    mock_agent.commit_memory.assert_called_once()
    called_arg = mock_agent.commit_memory.call_args[0][0]
    assert called_arg["type"] == "str_order"
    assert called_arg["order_id"] == order_id
    assert called_arg["payload"]["amount"] == 100.50
    assert called_arg["payload"]["to"] == "12345-6"
    assert called_arg["payload"]["reason"] == "Payment"

def test_upload_spb_file():
    mock_agent = MagicMock()
    config = SPBConfig()
    bridge = BfiBridge(mock_agent, config)

    file_content = b"test content"
    filename = "test.xml"

    upload_id = bridge.upload_spb_file(file_content, filename)

    assert len(upload_id) == 16
    mock_agent.commit_memory.assert_called_once()
    called_arg = mock_agent.commit_memory.call_args[0][0]
    assert called_arg["type"] == "spb_upload"
    assert called_arg["upload_id"] == upload_id
    assert called_arg["filename"] == filename
    assert called_arg["size"] == len(file_content)

@patch("substrate_933_bfi_bridge.time.sleep", side_effect=InterruptedError)
@patch("substrate_933_bfi_bridge.requests.get")
def test_listen_pix_events(mock_get, mock_sleep):
    mock_agent = MagicMock()
    config = SPBConfig()
    bridge = BfiBridge(mock_agent, config)

    mock_response = MagicMock()
    mock_response.ok = True
    mock_response.json.return_value = {
        "events": [
            {"id": "ev1", "type": "payment_received"},
            {"id": "ev2", "type": "payment_sent"}
        ]
    }
    mock_get.return_value = mock_response

    mock_callback = MagicMock()

    # Expected to raise InterruptedError to break the infinite loop
    with pytest.raises(InterruptedError):
        bridge.listen_pix_events(mock_callback)

    assert mock_callback.call_count == 2
    assert mock_agent.commit_memory.call_count == 2

    first_call_arg = mock_agent.commit_memory.call_args_list[0][0][0]
    assert first_call_arg["type"] == "pix_event"
    assert first_call_arg["data"]["id"] == "ev1"

def test_reconcile_clearing():
    mock_agent = MagicMock()
    config = SPBConfig()
    bridge = BfiBridge(mock_agent, config)

    # Should not raise any errors
    bridge.reconcile_clearing()
