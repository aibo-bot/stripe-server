# Stripe Server

A general-purpose stripe webhook server built for aibo. This server has been tailored to aibo, but feel free to learn how it works internally
to make your own!

It uses no IPC, as its only use is to make database changes and DM users which can be done through HTTP requests.