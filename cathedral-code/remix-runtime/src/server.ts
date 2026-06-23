import { Hono } from 'hono';

const app = new Hono();

app.get('/health', (c) => c.text('OK'));

app.post('/api/compile', async (c) => {
    const body = await c.req.json();
    return c.json({
        success: true,
        abi: [],
        bytecode: "0x1234",
        bytecode_hash: "0xhash",
        ast: {}
    });
});

app.post('/api/debug/session', async (c) => {
    return c.text("dummy-session-id");
});

app.post('/api/debug/step', async (c) => {
    const body = await c.req.json();
    return c.json({
        session_id: body.session_id,
        current_step: body.step,
        call_stack: [],
        locals: {},
        storage: {}
    });
});

app.post('/api/deploy', async (c) => {
    return c.json({
        success: true,
        contract_address: "0xAddress",
        transaction_hash: "0xTxHash",
    });
});

app.post('/api/plugin/call', async (c) => {
    return c.json({ status: "ok" });
});

export default app;
