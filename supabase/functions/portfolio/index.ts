import { createClient } from 'https://cdn.jsdelivr.net/npm/@supabase/supabase-js/+esm'



let ErrorResponse = (message) =>{
  return new Response(String(message), { status: 500 })
}

Deno.serve(async (_req) => {

  const supabaseUrl = Deno.env.get('SUPABASE_URL') ?? ''
  const supabaseKey = Deno.env.get('SUPABASE_ANON_KEY') ?? ''
  const apiKey = Deno.env.get('POLYGON_API_KEY')
  const polygon_url = Deno.env.get('POLYGON_URL')



  const supabase = createClient(supabaseUrl, supabaseKey,{ global: { headers: { Authorization: _req.headers.get('Authorization')! } } })
  let { data: update_times, error } = await supabase
      .from('stocks')
      .select('update_at, symbol, close_price')

  if (error) {
    return ErrorResponse(error.message);
  }

  for (let index = 0; index < update_times.length; index++) {
    let element = update_times[index];
    const symbol = element.symbol;
    const yesterday_price = element.close_price;
    const update_at =new Date( element.update_at);
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    if ( update_at.getTime() - today.getTime() >= 0){
      continue;
    }


    const res = await fetch(`${polygon_url}v2/aggs/ticker/${symbol}/prev?adjusted=true&apiKey=${apiKey}`);
    const json = await res.json();
    if (res.status >=400 ){
      return ErrorResponse(res.statusText);
    }

    const updateJson = {
      close_price : json['results'][0]['c'],
      open_price :  json['results'][0]['o'],
      highest_price : json['results'][0]['h'],
      lowest_price : json['results'][0]['l'],
      yesterday_price : yesterday_price,
      update_at : today
    }


    await supabase.from('stocks')
        .update(updateJson)
        .eq('symbol',symbol);

  }





  let { data: holdings, err_stocks } = await supabase
      .from('stocks')
      .select('*');



  if (err_stocks) {
    return ErrorResponse(err_stocks.message);
  }


  let balance = 0;

  for (let i = 0; i < holdings.length; i++) {
    balance += holdings[i].owned * holdings[i].close_price;
  }

  const {data: history, err_history } = await supabase.rpc('find_or_insert_history', {
    p_balance: balance
  });

  if (err_history) {
    console.error('Error executing RPC function:', error);
    return ErrorResponse(err_history.message);
  }


  return new Response(JSON.stringify({holdings,history}), {
    headers: { 'Content-Type': 'application/json' },
    status: 200,
  })

})


/* To invoke locally:

  1. Run `supabase start` (see: https://supabase.com/docs/reference/cli/supabase-start)
  2. Make an HTTP request:

  curl -i --location --request POST 'http://127.0.0.1:54321/functions/v1/portfolio' \
    --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0' \
    --header 'Content-Type: application/json' \
    --data '{"name":"Functions"}'

*/
