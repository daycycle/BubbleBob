/*
Copyright 2024 David-Alexandre Yana
Licensed under the terms in LICENSE.txt
*/

const invoke = window.__TAURI__.invoke

const licenseModal = new bootstrap.Modal(document.getElementById("license-modal"), {keyboard: false});

window.onload = function() {
    localStorage.setItem("gasSelection", "Air");
    localStorage.setItem("unitsSystem", "Metric");
    if (localStorage.getItem("userAcceptedLicense") == "true") {
        updateOperatingDepth();
    } else {
        licenseModal.show();
    }
}

function acceptLicense() {
    localStorage.setItem("userAcceptedLicense", true);
    licenseModal.hide();
    updateOperatingDepth();
}

function exit() {
    invoke('exit');
}

function setPresetGas(gas) {
    localStorage.setItem("gasSelection", gas);
    updateOperatingDepth();
}

function setUnitsSystem(unitsSystem) {
    localStorage.setItem("unitsSystem", unitsSystem);
    updateOperatingDepth();
}

function updateOperatingDepth() {
    document.getElementById('content').innerHTML = ""
    switch (localStorage.getItem("gasSelection")) {
        case "Air":
            invoke('get_operating_depth', {
                    mixer: 'Air', unitsSystem: localStorage.getItem("unitsSystem")
            })
                .then((response) => displayOperatingDepth(response))
                .catch((error) => displayOperatingDepthError(error))
            break;
        case "Heliox":
            invoke('get_operating_depth', {
                    mixer: {
                        Heliox: { dioxygen: parseFloat(document.getElementById('heliox-dioxygen').value) / 100 || 0 }
                    },
                    unitsSystem: localStorage.getItem("unitsSystem")
            })
                .then((response) => displayOperatingDepth(response))
                .catch((error) => displayOperatingDepthError(error))
            break;
        case "Nitrox":
            invoke('get_operating_depth', {
                mixer: {
                    Nitrox: { dioxygen: parseFloat(document.getElementById('nitrox-dioxygen').value) / 100 || 0 }
                },
                unitsSystem: localStorage.getItem("unitsSystem")
            })
                .then((response) => displayOperatingDepth(response))
                .catch((error) => displayOperatingDepthError(error))
            break;
        case "Trimix":
            invoke('get_operating_depth', {
                mixer: {
                    Trimix: {
                        dinitrogen: parseFloat(document.getElementById('trimix-dinitrogen').value) / 100 || 0,
                        dioxygen: parseFloat(document.getElementById('trimix-dioxygen').value) / 100 || 0,
                        helium: parseFloat(document.getElementById('trimix-helium').value) / 100 || 0,
                    }
                },
                unitsSystem: localStorage.getItem("unitsSystem")
            })
                .then((response) => displayOperatingDepth(response))
                .catch((error) => displayOperatingDepthError(error))
            break;
    }
}

function displayOperatingDepth(response) {
    document.getElementById('content').innerHTML =
    `<div class="col mx-5">
        <div class="row">
            <div class="card text-center mt-4 p-3">
                <div class="card-body">
                    <h3 class="card-title">` + response.minimum + `</h3>
                    <p class="card-text">Minimum</p>
                </div>
            </div>
            <div class="card text-center mt-4 p-3">
                <div class="card-body">
                    <h3 class="card-title">` + response.maximum + `</h3>
                    <p class="card-text">Maximum</p>
                </div>
            </div>
        </div>
        <div class="row"><div>
    </div>`
}

function displayOperatingDepthError(error) {
    let message;
    let divClass;
    if (error.GasMixError) {
        divClass = "alert-warning";
        switch (error.GasMixError) {
            case "GasMixProportionsNotValid":
                message = "The total of the gas proportions in the mix should be close to 100%.";
                break;
            case "NoDioxygenInGasMix":
                message = "The mix must contain dioxygen.";
                break;
        }
    } else if (error.GasMixerError) {
        divClass = "alert-danger";
        switch (error.GasMixerError) {
            case "NegativeProportion":
                message = "A gas cannot have a negative proportion in the mix.";
                break;
            case "ProportionAboveMaximum":
                message = "A gas cannot have a proportion greater than 100%.";
                break;
        }
    } else if (error.OperatingDepthError) {
        divClass = "alert-danger";
        switch (error.OperatingDepthError) {
            case "NoMinimumOperatingPressure":
                message = "The gas mix has no minimum operating pressure. Make sure the mix contains at least 1 gas with a minimum limit.";
                break;
            case "NoMaximumOperatingPressure":
                message = "The gas mix has no maximum operating pressure. Make sure the mix contains at least 1 gas with a maximum limit.";
                break;
            case "NoValidRange":
                message = "The mix doesn't have a valid operating range.";
                break;
        }
    } else if (error.LimitsError) {
        divClass = "alert-danger";
        switch (error.LimitsError) {
            case "NoMinimumDioxygenLimit":
                message = "There must be a minimum limit for dioxygen.";
                break;
            case "NoMaximumDioxygenLimit":
                message = "There must be a maximum limit for dioxygen.";
                break;
        }
    } else {
        message = error;
        divClass = "alert-danger";
    }
    document.getElementById('content').innerHTML = `<div class="alert ` + divClass + ` mx-5 mt-4" role="alert">` + message + `</div>`
}
