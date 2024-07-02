import React, { useState } from 'react';
import { idlFactory as token_idl, canisterId as token_id } from '../../declarations/token_wallet_backend';
import { Actor, HttpAgent } from '@dfinity/agent';

const agent = new HttpAgent();
const token = Actor.createActor(token_idl, { agent, canisterId: token_id });

function App() {
  const [balance, setBalance] = useState(0);
  const [fromAddress, setFromAddress] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState(0);

  const getBalance = async () => {
    const balance = await token.get_balance(fromAddress);
    setBalance(balance || 0);
  };

  const sendToken = async () => {
    await token.send_token(fromAddress, toAddress, Number(amount));
    getBalance();
  };

  return (
    <div className="container">
      <h1>Token Wallet</h1>
      <input
        type="text"
        placeholder="From Address"
        value={fromAddress}
        onChange={(e) => setFromAddress(e.target.value)}
      />
      <input
        type="text"
        placeholder="To Address"
        value={toAddress}
        onChange={(e) => setToAddress(e.target.value)}
      />
      <input
        type="number"
        placeholder="Amount"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={sendToken} disabled={!fromAddress | !toAddress | !amount}>Send Token</button>
      <button onClick={getBalance} disabled={!fromAddress}>Get Balance</button>
      <p>Balance: {balance}</p>
    </div>
  );
}

export default App;
