import pytest
from substrate_931_interfold_bridge import E3Adapter, CiphernodeClient, VerifiableRelease, ConfidentialOrchestrator

def test_e3_creation():
    adapter = E3Adapter()
    e3_id = adapter.create_e3("test_logic")
    assert e3_id in adapter.active_e3s
    assert adapter.active_e3s[e3_id].logic == "test_logic"

    adapter.submit_input(e3_id, "input_1")
    adapter.submit_input(e3_id, "input_2")
    assert len(adapter.active_e3s[e3_id].inputs) == 2

    result = adapter.execute(e3_id)
    assert result["logic"] == "test_logic"
    assert result["input_count"] == 2
    assert result["computed"] is True

    adapter.destroy_e3(e3_id)
    assert e3_id not in adapter.active_e3s

def test_ciphernode_threshold():
    client = CiphernodeClient(threshold=3, total_nodes=5)
    e3_id = "test-e3"

    committee = client.form_committee(e3_id)
    assert len(committee) == 5
    assert "node-0" in committee

    decrypted = client.request_threshold_decryption(e3_id, {"data": "encrypted"})
    assert decrypted["decrypted"] is True
    assert decrypted["data"] == {"data": "encrypted"}

def test_verifiable_release():
    release = VerifiableRelease()
    e3_id = "test-e3"

    # Valid proof
    success = release.verify_and_release(e3_id, "result_data", {"valid": True})
    assert success is True
    assert release.released_results[e3_id] == "result_data"

    # Invalid proof
    success = release.verify_and_release("e3-invalid", "bad_data", {"valid": False})
    assert success is False
    assert "e3-invalid" not in release.released_results

def test_orchestrator_flow():
    orchestrator = ConfidentialOrchestrator()
    inputs = ["bid_1", "bid_2", "bid_3"]
    logic = "auction_clearing"

    result = orchestrator.run_confidential_computation(logic, inputs)

    assert result is not None
    assert result["decrypted"] is True
    assert result["data"]["logic"] == "auction_clearing"
    assert result["data"]["input_count"] == 3
    assert result["data"]["computed"] is True
