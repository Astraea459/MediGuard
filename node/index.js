// dapp/node/index.js

const express = require('express');
const bodyParser = require('body-parser');
const axios = require('axios');

const app = express();
const port = 3000;

app.use(bodyParser.json());

const SECRET_REST_URL = 'http://localhost:1317';

app.post('/addHealthRecord', async (req, res) => {
    try {
        const healthRecord = req.body;
        const response = await axios.post(`${SECRET_REST_URL}/wasm/execute`, {
            contract: 'your_contract_address', // Replace with your actual contract address
            msg: {
                add_health_record: {
                    patient: healthRecord.patient,
                    medical_history: healthRecord.medical_history,
                    test_results: healthRecord.test_results,
                    prescriptions: healthRecord.prescriptions,
                },
            },
            sender: 'your_sender_address', // Replace with your actual sender address
        });

        res.json({ success: true, data: response.data });
    } catch (error) {
        console.error(error);
        res.status(500).json({ success: false, error: 'Internal server error' });
    }
});

app.get('/getHealthRecords', async (req, res) => {
    try {
        const response = await axios.get(
            `${SECRET_REST_URL}/wasm/contract/your_contract_address/store`,
        ); // Replace with your actual contract address

        res.json({ success: true, data: response.data });
    } catch (error) {
        console.error(error);
        res.status(500).json({ success: false, error: 'Internal server error' });
    }
});

app.listen(port, () => {
    console.log(`Server is running at http://localhost:${port}`);
});
