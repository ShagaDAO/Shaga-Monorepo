Shaga is a peer-2-peer Cloud Gaming protocol that leverages Solana, Sunshine & Moonlight.

This repo contains the ShagaJoe Program, the Solana logic that is based on @kamikazejoe.

The TLDR of how the protocol works is this:
Shaga modifies the phase 1 of the Moonlight protocol (an open-source NVIDIA GameStream Implementation https://games-on-whales.github.io/wolf/stable/protocols/http-pairing.html#_phase_1) by changing the way the PIN to pair the Server & Client is shared.

In the vanilla implementation, the PIN needs to be inserted manually for the pairing to happen. In Shaga, we leverage Solana's ED25519 Keypairs and map them to X25519 Keypairs, then the Moonlight-Client uses its Private X25519 Key and the Server's Public X25519 Key to Encrypt the PIN, then sends it using this http request:
"
        String getCert = http.executeShagaPairingCommand("phrase=getservercert&salt=" +
                        bytesToHex(salt) + "&clientcert=" + bytesToHex(pemCertBytes) +
                        "&encryptedPin=" + hexEncryptedPin + "&publicKey=" + publicKeyBase58,
                false);
"

The Moonlight-Client gets the Server's IP_Address & PublicKey by fetching from the ShagaJoe Program a list of all the session_accounts available, uses it to ping the IP_Addresses to get a latency measure, then when a session to join has been chosen, it invokes the start_rental instruction and pays rent in advance.

The Sunshine Server then, handles the http request and uses a POST to send the EncryptedPIN & PublicKey received to the server's frontend, where the decryption happens in Typescript.

On the Frontend, the server uses a websocket connection to get updates from the Session_Account that it's created when the Server wants to start Lending the GamingPC, to get updates on the state of its account, and when the server gets an update that the client has paid rent, the frontend sends the decrypted PIN to the backend and the pairing proceeds business as usual, according to https://games-on-whales.github.io/wolf/stable/protocols/http-pairing.html#_phase_1.

Once the session terminates, the Sunshine Server's frontend that still has an open websocket to the relevant session_account, sends a command to the server's backend to unpair all the clients.

Here are the links to the Moonlight & Sunshine forks that implement a burner solana wallet encrypted locally (shaga-client & shaga-server git branches):
- https://github.com/chat-grp/moonlight-android/tree/shaga-client
- https://github.com/chat-grp/Sunshine/tree/shaga-server

This is a very rudimentary implementation and the work started on the 10th of September to submit the Available project to the Solana Hyperdrive Hackathon.

Infinite gratitude to the folks in the Sunshine-Lizardbyte's discord and to A.Beltramo from the Games on Whales discord for their precious tips on the integration points on Sunshine & Moonlight.

Also this wouldn't have been possible without @GabrielePicco & @JonasHahn help on the solana program side.
