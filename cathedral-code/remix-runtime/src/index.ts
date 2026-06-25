import { serve } from '@hono/node-server';
import app from './server';

const port = parseInt(process.env.PORT || '3000');
console.log(`Starting Remix Runtime on port ${port}`);

serve({
  fetch: app.fetch,
  port
});
